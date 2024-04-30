# <img src="./appearance/blaze_logo.png" width="100px"> database
Welcome to <strong>Blaze</strong> — the brand new relational key-value database which utilizes a unique programming language to provide flexible, easy storage modeling, data management, and operation. 
And of course it's <strong>blazingly fast</strong>
<p>The main programming language under the hood is <span style="color:#fc5604">Rust</span>.</p>


## ✨ Blaze Language Syntax (currently uncompleted)
```ruby
package lib;

enum Gender: str {
    male, 
    female, 
    other,
    unspecified
};

table countries {
    name: str <=50,
    alpha2: str 2,
    alpha3: str 3,
    geolocation: geo,
};

table accounts: uuid {
    name: str <=30 = format("User{}", self.id),
    bio: str <=200,
    password: str,
    gender: Gender = Gender.unspecified,
    age: int >0 <100,
    country: &countries?,
    created_at: datetime = "now";
};

function get_population_of(country_name: str): int {
    fin country_id = &countries.{self.country}.id; 
    count(&accounts.{self.country=country_id})
};
