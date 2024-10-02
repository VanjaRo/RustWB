use futures::future::select_all;
use linkify::{LinkFinder, LinkKind};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::{channel, unbounded_channel, Receiver, UnboundedSender};

#[derive(Clone, Default)]
pub struct Config {
    pub concurrent_requests: Option<usize>,
}

pub struct Page {
    pub url: String,
    pub body: String,
}

pub struct Crawler {
    config: Config,
}

impl Crawler {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn run(&mut self, site: String) -> Receiver<Page> {
        let rate_limit = self.config.concurrent_requests.unwrap_or(1);

        let (page_snd, page_rcv) = channel(rate_limit);

        tokio::spawn(async move {
            let (link_snd, mut link_rcv) = unbounded_channel();
            let visited = Arc::new(Mutex::new(HashSet::new()));
            let mut visit_fut = vec![Box::pin(Self::visit(
                site,
                visited.clone(),
                link_snd.clone(),
            ))];
            loop {
                let (page, _, mut rem) = select_all(visit_fut).await;
                if let Some(page) = page {
                    page_snd.send(page).await.unwrap();
                }

                // no links to process
                if rem.is_empty() && link_rcv.is_empty() {
                    break;
                }

                // fits the rate and contains smth to process
                while !link_rcv.is_empty() && rem.len() < rate_limit {
                    let new_link = link_rcv.recv().await.unwrap();
                    rem.push(Box::pin(Self::visit(
                        new_link,
                        visited.clone(),
                        link_snd.clone(),
                    )))
                }
                visit_fut = rem;
            }
        });

        page_rcv
    }

    async fn visit(
        site: String,
        visited: Arc<Mutex<HashSet<String>>>,
        link_snd: UnboundedSender<String>,
    ) -> Option<Page> {
        if !visited.lock().unwrap().insert(site.clone()) {
            return None;
        }
        let content = reqwest::get(site.clone())
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let mut finder = LinkFinder::new();
        finder.kinds(&[LinkKind::Url]);
        let links = finder.links(&content).map(|l| l.as_str().to_string());
        for link in links {
            link_snd.send(link.clone()).unwrap();
        }
        Some(Page {
            url: site,
            body: content,
        })
    }
}

async fn save_file(file_name: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::create(file_name).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: wget <url>");
        return;
    }

    let url = args[1].clone();
    let config = Config {
        concurrent_requests: Some(4), // Example: 4 concurrent requests
    };
    let mut crawler = Crawler::new(config);
    let mut page_rcv = crawler.run(url);

    while let Some(page) = page_rcv.recv().await {
        let file_name = page.url.replace(['/', ':', '?', '&', '=', '#'], "_") + ".html";
        if let Err(e) = save_file(&file_name, &page.body).await {
            eprintln!("Error saving file {}: {}", file_name, e);
        } else {
            println!("Saved file: {}", file_name);
        }
    }
}
