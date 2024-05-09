#!/usr/bin/perl

use threads;
use lib 'SharedData';
use strict;
use warnings;
use LWP::UserAgent;
use HTTP::Request;
use Text::CSV;
use URI::URL;
use DateTime;
use HTML::Entities;
use WWW::RobotRules;

use constant MAX_REQUESTS_PER_MINUTE => 100;

# Create a thread to reset the request counts every minute
threads->create(sub {
    while (1) {
        sleep(60);
        {
            lock(%SharedData::request_counts);
            %SharedData::request_counts = ();
        }
    }
})->detach;

sub run {
    print "Running run subroutine\n";

    my $robot_rules = WWW::RobotRules->new('MyRobot/0.1');
    my $seed_page = $ENV{SEED_PAGE} // 'https://en.wikipedia.org/wiki/Main_Page';
    {
        lock(%SharedData::visited_urls);
        lock(@SharedData::shared_queue);
        if (!exists $SharedData::visited_urls{$seed_page} && !grep { $_ eq $seed_page } @SharedData::shared_queue) {
            push @SharedData::shared_queue, $seed_page;
            # print "\tQueue: @SharedData::shared_queue\n";
            # print "\tVisited URLs: ", join(", ", keys %SharedData::visited_urls), "\n";
            # print "\tEnqueued $seed_page\n";
        }
    }
    my $limit = $ENV{QUERY_LIMIT} // 10;
    my $timeout = $ENV{TIMEOUT} // 1;
    my @blacklist = ('mailto:', 'meta.', 'Help:', 'Special:', 'File:', 'Talk:', 'User:', 'Wikipedia:', 'Template:', 'Portal:', 'Category:', 'Thread:', 'Index:', 'MediaWiki:', 'Book:', 'Draft:', 'Education Program:', 'TimedText:', 'Module:', 'Gadget:', '.php');

    my $output_dir = $ENV{OUTPUT_DIR} // 'data';

    my $csv = Text::CSV->new({ binary => 1, eol => $/ });
    open my $fh, '>:encoding(utf8)', "$output_dir/scraped_data.csv" or die "$output_dir/scraped_data.csv: $!";
    $csv->print($fh, ['Image Name', 'Image URL', 'Source', 'Scraped at', 'Page Title']);

    my $visited = Text::CSV->new({ binary => 1, eol => $/ });
    open my $fh_v, '>:encoding(utf8)', "$output_dir/visited_sites.csv" or die "$output_dir/visited_sites.csv: $!";
    $visited->print($fh_v, ['Page Title', 'Page URL', 'Visited at', 'Status']);

    my $queued = Text::CSV->new({ binary => 1, eol => $/ });
    open my $fh_q, '>:encoding(utf8)', "$output_dir/queued_sites.csv" or die "$output_dir/queued_sites.csv: $!";
    $queued->print($fh_q, ['Link text', 'Page URL', 'Referrer', 'Queued at']);

    my $ua = LWP::UserAgent->new;
    $ua->agent("Mozilla/5.0");
    $ua->timeout(10);

    # print "Queue: @SharedData::shared_queue\n";
    # print "Visited URLs: ", join(", ", keys %SharedData::visited_urls), "\n";
    # print "Limit: $limit\n";
    while (@SharedData::shared_queue and keys %SharedData::request_counts < $limit) {
        my $url;
        {
            lock(@SharedData::shared_queue);
            $url = shift @SharedData::shared_queue;
            print "Dequeued $url\n";
        }

        # Skip if already visited
        next if exists $SharedData::visited_urls{$url};

        # Check if URL is allowed to be crawled
        if (!$robot_rules->allowed($url)) {
            print "Skipping $url due to robots.txt rules\n";
            next;
        }

        # Skip non-HTTP(S) URLs
        my $uri = URI->new($url);
        if ($uri->scheme !~ /^https?$/) {
            print "Skipping non-HTTP(S) URL: $url\n";
            next;
        }

        # Rate limiting
        my $domain = $uri->host;
        {
            lock(%SharedData::request_counts);
            my $count = $SharedData::request_counts{$domain} // 0;
            if ($count >= MAX_REQUESTS_PER_MINUTE) {
                print "Rate limit exceeded for $domain, sleeping for 60 seconds\n";
                sleep(60);
            }
            $SharedData::request_counts{$domain} = $count + 1;
        }

        # Mark as visited
        {
            lock(%SharedData::visited_urls);
            print "Visiting $url\n";
            $SharedData::visited_urls{$url} = 1;
        }

        # Make request
        my $req = HTTP::Request->new(GET => $url);
        my $res = $ua->request($req);
        my $scraping_time = DateTime->now->strftime('%Y-%m-%d %H:%M:%S');

        if ($res->is_success) {
            my $content = $res->content;

            # my @image_urls = $content =~ /<img[^>]*src="(.*?)"/g;
            my @image_urls = $content =~ /<img[^>]*src="(.*?(\.jpg|\.jpeg|\.png))"/ig;
            # my @image_urls = $content =~ /<img[^>]*class="[^"]*(x5yr21d xu96u03 x10l6tqk x13vifvy x87ps6o xh8yej3)[^"]*"[^>]*src="(.*?\.(jpg|jpeg|png))"/ig;

            my ($title) = $content =~ /<title>(.*?)<\/title>/s;
            $visited->print($fh_v, [$title, $url, $scraping_time, "200 OK"]);

            for my $j (0 .. $#image_urls) {
                my $image_url = URI::URL->new($image_urls[$j], $url)->abs;
                my ($image_name) = $image_url =~ /([^\/]+)$/;
                $csv->print($fh, [$image_name, $image_url, $url, $scraping_time, $title]);
            }

            # Extract new URLs and add them to the queue
            # while ($content =~ /<a[^>]*href="(.*?)">(.*?)<\/a>/g) {
            # while ($content =~ /<a[^>]*href="([^"#]*?)">(.*?)<\/a>/g) {
            while ($content =~ /<a[^>]*href="([^"#]*?)">(.*?)<\/a>/g) {

                my $new_url = $1;
                my $link_text = decode_entities($2);
                $link_text =~ s/<[^>]*>//g;

                print "\tFound $new_url\n";

                # Make URL absolute if it's relative
                $new_url = URI::URL->new($new_url, $url)->abs;

                # Skip if URL contains any blacklisted keyword
                next if grep { $new_url =~ /\Q$_\E/ } @blacklist;

                # Skip if already visited or queued
                next if exists $SharedData::visited_urls{$new_url} or grep { $_ eq $new_url } @SharedData::shared_queue;

                print "\tAdding $new_url to queue\n";
                {
                    lock(@SharedData::shared_queue);
                    my $new_url_str = $new_url->as_string;
                    if (ref($new_url_str)) {
                        warn "Invalid value for shared scalar: $new_url_str";
                    } else {
                        push @SharedData::shared_queue, $new_url_str;
                    }
                }
                $queued->print($fh_q, [$link_text, $new_url, $url, $scraping_time]);
            }

        } else {
            if ($res->status_line =~ /read timeout/) {
                print "Timeout occurred while trying to get $url, skipping.\n";
            } else {
                print "Couldn't get $url ", $res->status_line, "\n";
            }
            $visited->print($fh_v, ['', $url, $scraping_time, $res->status_line]);
        }
        print "Sleeping for $timeout seconds\n";
        sleep($timeout); # Don't be evil --- be polite ^^
    }

    close $fh;
    close $fh_v;
    close $fh_q;
    print "Finished run subroutine\n";
}
1;