#!/usr/bin/perl

use strict;
use warnings;
use Text::CSV;
use MongoDB;
use Config::Simple;

# Config
my $cfg = new Config::Simple('./../.env');
my $mongo_url = $cfg->param('MONGO_URL');
my $client = MongoDB::MongoClient->new(host => $mongo_url);
my $db = $client->get_database('spider_web');

for my $worker_dir (glob "data/*") {
    next unless -d $worker_dir;
    print "Uploading $worker_dir ...";
    process_csv($worker_dir);
    print " DONE\n";
}

sub process_csv {
    my ($worker_dir) = @_;
    my $csv = Text::CSV->new({ binary => 1 }) or die "Cannot use CSV: " . Text::CSV->error_diag();
    
    open my $fh_visited, "<", "$worker_dir/visited_sites.csv" or die "$worker_dir/visited_sites.csv: $!";
    $csv->getline($fh_visited);
    while (my $row_visited = $csv->getline($fh_visited)) {
        my ($page_title, $page_url, $visited_at, $status) = @$row_visited;
        $db->get_collection('visited_sites')->insert_one({ page_title => $page_title, page_url => $page_url, visited_at => $visited_at, status => $status });
    }
    close $fh_visited;

    open my $fh_scraped, "<", "$worker_dir/scraped_data.csv" or die "$worker_dir/scraped_data.csv: $!";
    $csv->getline($fh_scraped);
    while (my $row_scraped = $csv->getline($fh_scraped)) {
        my ($image_name, $image_url, $source, $scraped_at, $page_title) = @$row_scraped;
        $db->get_collection('scraped_data')->insert_one({ image_name => $image_name, image_url => $image_url, source => $source, scraped_at => $scraped_at, page_title => $page_title });
    }
    close $fh_scraped;

    open my $fh_queued, "<", "$worker_dir/queued_sites.csv" or die "$worker_dir/queued_sites.csv: $!";
    $csv->getline($fh_queued);
    while (my $row_queued = $csv->getline($fh_queued)) {
        my ($link_text, $page_url, $referrer, $queued_at) = @$row_queued;
        $db->get_collection('queued_sites')->insert_one({ link_text => $link_text, page_url => $page_url, referrer => $referrer, queued_at => $queued_at });
    }
    close $fh_queued;
}