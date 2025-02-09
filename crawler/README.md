# Crawler

## What's this?

This Perl script is a simple "polite" web crawler designed to traverse and gather data from a specified starting webpage.
It identifies image URLs and their associated names from each page it visits and records this information, along with the page title and the timestamp of the scraping event, into a CSV file named `scraped_data.csv`.
The script then uses a PageRank-like method to find all pages linked from the current page and adds them to a queue for future visits.
Additionally, it logs the URLs of visited and queued sites, along with some additional metadata, to `visited_sites.csv` and `queued_sites.csv` respectively.

> [!TIP]
> The script can be limited to visiting a maximum number of URLs by changing the value of the `$limit` variable. It also maintains a blacklist of keywords to avoid queuing certain types of URLs.

> [!WARNING]
> Currently, the script does not implement any form of prioritization of URLs.

## Architecture

![WebCrawlerArchitecture svg](https://github.com/carlobortolan/b1o.co/assets/106114526/41239a82-d27a-4506-a612-49ea21c79cfc)

1. **Scheduler**: The [`Scheduler.pl`](Scheduler.pl) script creates multiple worker threads that each run the run subroutine from [`Runner.pl`](Runner.pl) which allows for multiple worker threads to download web pages in parallel.

2. **Queue**: The [`SharedData.pm`](SharedData.pm) package provides a shared queue ([`@SharedData::shared_queue`](SharedData.pm)) as a queue of URLs to be downloaded. Each worker thread adds its seed page to the queue and then runs the run subroutine, which presumably removes URLs from the queue as it processes them.

3. **Multi-threaded runner**: The [`Runner.pl`](Runner.pl) script contains the main loop of the crawler, which fetches URLs from the queue, downloads them and enqueues new URLs found on the page.

4. **Storage**: The [`Connector.pl`](Connector.pl) persists the scraped data to a remote MongoDB collection.

> ![NOTE]
> It's up to the run subroutine (3.) to actually write the downloaded pages to the temporary data directory before being persistantly stored (4.).

## Config

1. Install the necessary dependencies:

   ```perl
   cpan Text::CSV
   cpan LWP::Protocol::https
   cpan MongoDB::MongoClient
   cpan Config::Simple
   ```

> [!NOTE]
> You might have to run those commands as a user with admin privileges (e.g., `sudo cspan Text::CSV`)

2. Run the script:

   ```perl
   perl downloader.pl
   ```

---

© Carlo Bortolan

> Carlo Bortolan &nbsp;&middot;&nbsp;
> GitHub [carlobortolan](https://github.com/carlobortolan) &nbsp;&middot;&nbsp;
> contact via [carlobortolan@gmail.com](mailto:carlobortolan@gmail.com)
