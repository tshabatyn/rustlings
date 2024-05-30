// Copyright 2024 Taras Shabatyn
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     https://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// use colored::*;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait NewSummary {
    fn new_summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArrticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArrticle {
    fn summarize(&self) -> String {
        format!("{}, by {} (at: {})", self.headline, self.author, self.location)
    }
}
impl NewSummary for NewsArrticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}
impl NewSummary for Tweet {
    fn new_summarize(&self) -> String {
        format!("New Tweet by @{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("lavrov_the_🐴"),
        content: String::from("И го го-го. Please bring me to the Hague ⚖️ ."),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new new tweet: {}", tweet.new_summarize());

    let article = NewsArrticle {
        headline: String::from("🐧 Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh ❄️ "),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    // println!("New article available! {}", article.summarize().green());
    // println!("New article available! {}", console::style(article.summarize()).green());
    println!("New article available! {}", article.summarize());
    println!("New New article available! {}", article.new_summarize());
}