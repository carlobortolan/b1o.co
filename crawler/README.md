# Crawler

_"definitely necessary to bring out emacs and modify that perl script"_

## What's this?

This Perl script is a simple web scraper designed to traverse and gather data from a specified starting webpage.
It identifies image URLs and their associated names from each page it visits and records this information, along with the page title and the timestamp of the scraping event, into a CSV file named `scraped_data.csv`.
The script then uses a PageRank-like method to find all pages linked from the current page and adds them to a queue for future visits.
Additionally, it logs the URLs of visited and queued sites, along with some additional metadata, to `visited_sites.csv` and `queued_sites.csv` respectively.

> [!TIP]
> The script can be limited to visiting a maximum number of URLs by changing the value of the `$limit` variable. It also maintains a blacklist of keywords to avoid queuing certain types of URLs.

## Architecture (planned)

![WebCrawlerArchitecture svg](https://github.com/carlobortolan/b1o.co/assets/106114526/41239a82-d27a-4506-a612-49ea21c79cfc)

## Config

1. Install the necessary dependencies:

   ```perl
   cpan Text::CSV
   cpan install LWP::Protocol::https
   ```

> [!NOTE]
> You might have to run those commands as a user with admin privileges (e.g., `sudo cspan Text::CSV`)

2. Run the script:

   ```perl
   perl run.pl
   ```

---

© Carlo Bortolan

> Carlo Bortolan &nbsp;&middot;&nbsp;
> GitHub [carlobortolan](https://github.com/carlobortolan) &nbsp;&middot;&nbsp;
> contact via [carlobortolan@gmail.com](mailto:carlobortolan@gmail.com)
