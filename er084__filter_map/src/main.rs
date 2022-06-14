// https://youtu.be/4ucNNpxd5Q4?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo =match ceo {
            "" => None,
            name => Some(name.to_string())
        };        
        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    let all_the_ceos = company_vec
        .into_iter()
        .filter_map(|company| company.get_ceo())
        .collect::<Vec<String>>();

    println!("{:?}", all_the_ceos);
}
