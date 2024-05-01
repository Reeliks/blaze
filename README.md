# <img src="./appearance/blaze_logo.png" width="100px"> database
Welcome to <strong>Blaze</strong> — the brand new relational key-value database which utilizes a unique programming language to provide flexible, easy storage modeling, data management, and operation. 
And of course it's <strong>blazingly fast</strong>
<p>The main programming language under the hood is <span style="color:#fc5604">Rust</span>.</p>


## ✨ Blaze Language Syntax (currently uncompleted)
1. Manager.blz (used to raise the database with packages included)
```ruby 
manage (
    packages = "./packages",
    max_connections = 8,
    port = "6980",
    host = "127.0.0.1"
);

import users:all, animals:species;

inspect all;

attach "./data";

```
2. Basic Scheme and some declarations
```ruby
package scheme;

enum Gender: str {
    Male, 
    Female, 
    Other,
    Unspecified
};

enum TargetAudience: str {
    Kids,
    Everyone,
    Adults,
    Elderly
};

table countries {
    name: str <=50,
    alpha2: str 2,
    alpha3: str 3,
    geolocation: geo;
};

table accounts: uuid {
    name: str <=30 = format("User{}", self.id),
    bio: str <=200,
    password: str,
    gender: Gender = Gender.Unspecified,
    age: int >0 <100,
    country: &countries?,
    created_at: datetime = "now";
};


table products: uuid {
    title: str = "product",
    seller: &accounts,
    price: float >= 0,
    description: str,
    created_at: datetime = "now",
    sales_count: int >=0 ,
    audience: TargetAudience = TargetAudience.Everyone;
}  

table shopping_cart {
    account: &account!,
    added_products: &products[],
    last_update: datetime = "now";
};

event shopping_cart_update (&shopping_cart.added_products, "change") {
    &shopping_cart.last_update = "now";
};

function get_population_of(country_name: str): int {
    fin country_id = &countries.{self.name=country_name}.id; 
    count(&accounts.{self.country=country_id})
};

function get_total_cart_price(products: &products[]): float {
    mut total: float = 0;
    product of products {
        total += product.price;
    };
    total
}