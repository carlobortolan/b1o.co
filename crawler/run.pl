#!/usr/bin/perl
use strict;
use warnings;
use LWP::UserAgent;
use HTTP::Request;
use Text::CSV;
use URI::URL;
use DateTime;
use HTML::Entities;

my %visited_urls;
my @queued_urls = ('https://en.wikipedia.org/wiki/Main_Page');
my $limit = 10; # Query limit
my @blacklist = ('meta.', 'Help:', 'Special:', 'File:', 'Talk:', 'User:', 'Wikipedia:', 'Template:', 'Portal:', 'Category:', 'Thread:', 'Index:', 'MediaWiki:', 'Book:', 'Draft:', 'Education Program:', 'TimedText:', 'Module:', 'Gadget:');

my $csv = Text::CSV->new({ binary => 1, eol => $/ });
open my $fh, '>:encoding(utf8)', 'data/scraped_data.csv' or die "data/scraped_data.csv: $!";
$csv->print($fh, ['Image Name', 'Image URL', 'Source', 'Scraped at', 'Page Title']);

my $visited = Text::CSV->new({ binary => 1, eol => $/ });
open my $fh_v, '>:encoding(utf8)', 'data/visited_sites.csv' or die "data/visited_sites.csv: $!";
$visited->print($fh_v, ['Page Title', 'Page URL', 'Visited at', 'Status']);

my $queued = Text::CSV->new({ binary => 1, eol => $/ });
open my $fh_q, '>:encoding(utf8)', 'data/queued_sites.csv' or die "data/queued_sites.csv: $!";
$queued->print($fh_q, ['Link text', 'Page URL', 'Referrer', 'Queued at']);

my $ua = LWP::UserAgent->new;
$ua->agent("Mozilla/5.0");

while (@queued_urls and keys %visited_urls < $limit) {
    my $url = shift @queued_urls;

    # Skip if already visited
    next if exists $visited_urls{$url};

    # Mark as visited
    print "Visiting $url\n";
    $visited_urls{$url} = 1;

    # Make request
    my $req = HTTP::Request->new(GET => $url);
    my $res = $ua->request($req);
    my $scraping_time = DateTime->now->strftime('%Y-%m-%d %H:%M:%S');

    if ($res->is_success) {
        my $content = $res->content;

        my @image_urls = $content =~ /<img[^>]*src="(.*?)"/g;
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
            next if exists $visited_urls{$new_url} or grep { $_ eq $new_url } @queued_urls;

            print "\tAdding $new_url to queue\n";
            push @queued_urls, $new_url;
            $queued->print($fh_q, [$link_text, $new_url, $url, $scraping_time]);
        }

    } else {
        print "Couldn't get $url: ", $res->status_line, "\n";
        $visited->print($fh_v, ['', $url, $scraping_time, $res->status_line]);
    }
}

close $fh;