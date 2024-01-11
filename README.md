# bigbob
A chat client built on top of my tokenizer, toky with the help of python in the mix for gathering web results.

## Now how does it work?
When you run the program, youre given a prompt. Based on whatever you type in the prompt, <br />
the program first checks my very limited dataset for a response and prints it out. <br />
If it fails to get a response then it checks google for the most appropriate results related to the prompt <br />
and gets you a response.

## Let's try it
This is the responses i got for my questions.

```sh
>who are you?

I am a simple nlp testing agent created with nothing but the standard library and a tokenizer also built from scratch in rust

>what is your name?

Big Bob

>github 


GitHub: Let's build from here · GitHub : https://github.com/
GitHub : https://en.wikipedia.org/wiki/GitHub
What Is GitHub? A Beginner's Introduction to GitHub : https://kinsta.com/knowledgebase/what-is-github/
What Is GitHub? | Definition from TechTarget : https://www.techtarget.com/searchitoperations/definition/GitHub
GitHub - Apps on Google Play : https://play.google.com/store/apps/details?id=com.github.android&hl=en&gl=US
The GitHub Blog - Updates, ideas, and inspiration from GitHub ... : https://github.blog

```

## Installation
Here are the installation instructions. <br />
```sh
git clone https://github.com/bcbro2021/bigbob.git
cd bigbob
cargo run
```
