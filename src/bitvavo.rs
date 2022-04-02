use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Bid {
    price: String,
    quantity: String,
}

impl Bid {
    fn from_vec(vec: &Vec<String>) -> Bid {
        Bid {
            price: vec[0].clone(),
            quantity: vec[1].clone(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Ask {
    price: String,
    quantity: String,
}

impl Ask {
    fn from_vec(vec: &Vec<String>) -> Ask {
        Ask {
            price: vec[0].clone(),
            quantity: vec[1].clone(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Book {
    market: String,
    nonce: u64,
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
}

impl Book {
    fn get_bids(&self) -> Vec<Bid> {
        let mut bids: Vec<Bid> = Vec::new();
        for bid in self.bids.iter() {
            bids.push(Bid::from_vec(bid));
        }
        bids
    }

    fn get_asks(&self) -> Vec<Ask> {
        let mut asks: Vec<Ask> = Vec::new();
        for ask in self.asks.iter() {
            asks.push(Ask::from_vec(ask));
        }
        asks
    }
}

#[derive(Debug, Default)]
pub struct BitvavoClient {}

impl BitvavoClient {
    pub async fn get_book(&self, market: &str, depth: u32) -> Result<Book, reqwest::Error> {
        let book = reqwest::get(format!("https://api.bitvavo.com/v2/{}/book?depth={}", market, depth))
            .await?
            .json::<Book>()
            .await?;

        Ok(book)
    }
}
