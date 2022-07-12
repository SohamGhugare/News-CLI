# News CLI Application
This cli app prints out the latest news in India using the [Top Headlines Endpoint](https://newsapi.org/docs/endpoints/top-headlines) from [NewsAPI](https://newsapi.org)

## Installation

### Pre-requisites
You must have rust and cargo installed on your system.

### Installation procedure

1. Clone the repository using git <br>
`$ git clone https://github.com/SohamGhugare/News-CLI.git`

2. Go into the project directory <br>
`$ cd News-CLI`

3. Add your api key to the environment file. (You can get your api key [here](https://newsapi.org/)) <br>
`$ touch .env && echo "API_KEY='api_key'" >> .env`
**Note: replace the `api_key` with your api key string**

4. Build the project using cargo <br>
`$ cargo build`

5. Run the cli app <br>
**Linux:** `$ ./target/debug/news-cli` <br>
**Windows:** `$ .\target\debug\news-cli.exe`


