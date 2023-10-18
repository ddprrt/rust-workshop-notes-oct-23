use std::collections::BinaryHeap;
use url::Url;

#[derive(PartialEq, Eq, Debug)]
struct ShortestFirst(Url);

impl From<Url> for ShortestFirst {
    fn from(value: Url) -> Self {
        Self(value)
    }
}

impl PartialOrd for ShortestFirst {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShortestFirst {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = (other.0).as_str().len();
        let right = (self.0).as_str().len();

        left.cmp(&right)
    }
}

pub fn show_urls() {
    let raw_urls = vec![
        "https://www.youtube.com/",
        "https://fettblog.eu/",
        "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open",
    ];

    let mut urls = BinaryHeap::new();
    for url in raw_urls {
        urls.push(ShortestFirst(Url::parse(url).unwrap()));
    }

    while let Some(ShortestFirst(url)) = urls.pop() {
        println!("{:?}", url.as_str());
    }
}
