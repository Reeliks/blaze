# <img src="./appearance/blaze_logo.png" width="100px"> database
Welcome to <strong>Blaze</strong> — the brand new relational key-value database which utilizes a unique programming language to provide flexible, easy storage modeling, data management, and operation. 
And of course it's <strong>blazingly fast</strong>
<p>The main programming language under the hood is <span style="color:#fc5604">Rust</span>.</p>


## ✨ Blaze Language Syntax (currently uncompleted)
1. Manage file (configuration and attaching to a datafile are only available in this type of files)
```ruby 
manage (
    address      = "127.0.0.1:3306",
    cons_limit   = 250,
    con_lifetime = 120
);

import scheme[target];
attach()"./data.dblz";

```
2. A package with functions, events, enums, plans, and tables
```ruby
package scheme;

// Enum Members have their own IDs
// so you can access them without any changes but here.
enum Gender {
    Unspecified = [ 0, "unknown" ],
    Male        = [ 1, "boy"     ], 
    Female      = [ 2, "girl"    ], 
    Other       = [ 3, "other"   ],
};

// Snake_case is the standard
mut str = Gender.Male; // "boy"
fin best_gender: str = Gender[0]; // "unknown"

enum TargetAudience {
    Kids        = [ 0, lower(self [0]) ],
    Everyone    = [ 1, lower(self [1]) ],
    Adults      = [ 2, lower(self [2]) ],
    Elderly     = [ 3, lower(self [3]) ],
};

// Plan is a data structure that can be used for instances creation,
// and tables & lanes implementation
plan Geo {
    x: float, y: float
}

// Table ID's are created without explicit declaration,
// Exclamation mark means the field value must be unique
table countries {
    name:        !str[ <50 ],
    alpha2:      !str[ =2  ],
    alpha3:      !str[ =2  ],
    geolocation: Geo;
};

// db.something is a call to a database
table accounts: uuid {
    name:       str[ <=30    ]  = format("User{}", id),
    bio:        str[ <=200   ],
    password:   str[ >8      ],
    age:        int[ >0 <100 ],
    gender:     Gender          = Gender.Unspecified,
    created_at: datetime        = "now";
    country:    db.countries?,
};

plan Products {
    title:       str            = "product",
    created_at:  datetime       = "now",
    seller:      db.accounts,
    price:       float[>=0],
    description: str,
    sales_count: int[>-1],
    audience:    TargetAudience = TargetAudience.Everyone;
}  
table products(Products): uuid;

table shopping_cart {
    account:        !db.accounts,
    added_products: !arr[db.products],
    last_update:    timestamp[local]   = "now";
};

// Should be considered
event shopping_cart_updated("update", db.shopping_cart): x {
    x.last_update = "now";
};

function get_population_of(country_name: str): int {
    fin country_id = db.countries.(name=country_name).id; 
    count(db.accounts.(country=country_id))
};

function get_total_cart_price(products: Products): float {
    mut total: float = 0;
    product of products {
        total += product.price;
    };
    total
}
