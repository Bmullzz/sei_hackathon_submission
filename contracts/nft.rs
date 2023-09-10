#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NFT {
    pub id: u64,
    pub owner: String,
    pub metadata_uri: String,
}

pub struct Contract {
    nfts: Vec<NFT>,
    next_id: u64,
}

impl Contract {
    // Constructor to initialize the contract
    pub fn new() -> Self {
        Self {
            nfts: Vec::new(),
            next_id: 0,
        }
    }

    // Function to mint a new NFT
    pub fn mint(&mut self, owner: String, metadata_uri: String) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let nft = NFT {
            id,
            owner,
            metadata_uri,
        };
        self.nfts.push(nft);
        id
    }

    // Function to transfer an NFT from one owner to another
    pub fn transfer(&mut self, from: String, to: String, id: u64) -> bool {
        for nft in &mut self.nfts {
            if nft.id == id && nft.owner == from {
                nft.owner = to;
                return true;
            }
        }
        false
    }

    // Function to query details of an NFT using its ID
    pub fn get_nft(&self, id: u64) -> Option<&NFT> {
        self.nfts.iter().find(|&nft| nft.id == id)
    }
}

fn main() {
    // Example usage
    let mut contract = Contract::new();
    let nft_id = contract.mint("Alice".to_string(), "https://metadata.url".to_string());
    println!("Minted NFT with ID: {}", nft_id);
    
    let transfer_success = contract.transfer("Alice".to_string(), "Bob".to_string(), nft_id);
    if transfer_success {
        println!("NFT transferred to Bob");
    } else {
        println!("Transfer failed");
    }
    
    let nft = contract.get_nft(nft_id).unwrap();
    println!("NFT ID: {}, Owner: {}, Metadata: {}", nft.id, nft.owner, nft.metadata_uri);
}
