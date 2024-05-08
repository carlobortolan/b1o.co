#!/usr/bin/perl
use threads;
use strict;
use warnings;
use lib '.';
use SharedData;
require './run.pl';

my $num_threads = 5;
my @seed_pages = ('https://en.wikipedia.org/wiki/Main_Page', 'https://en.wikipedia.org/wiki/Perl', 'https://en.wikipedia.org/wiki/Python_(programming_language)', 'https://en.wikipedia.org/wiki/Java_(programming_language)', 'https://en.wikipedia.org/wiki/C_(programming_language)');
my $limit = 10;
my $timeout = 1;

# Create worker
for my $i (0..$num_threads-1) {
    threads->create(\&worker, $i+1, $seed_pages[$i], $limit, $timeout);
}

# Worker subroutine
sub worker {
    my ($worker_num, $seed_page, $limit, $timeout) = @_;
    my $dir = "./data/worker$worker_num";
    mkdir $dir unless -d $dir;
    $ENV{OUTPUT_DIR} = $dir;
    $ENV{SEED_PAGE} = $seed_page;
    $ENV{QUERY_LIMIT} = $limit;
    $ENV{TIMEOUT} = $timeout;
    print "Starting worker $worker_num\n";
    {
        lock(@SharedData::shared_queue);
        if (!exists $SharedData::visited_urls{$seed_page} && !grep { $_ eq $seed_page } @SharedData::shared_queue) {
            push @SharedData::shared_queue, $seed_page;
        }
    }
    run();
    print "Finished worker $worker_num\n";
}

# Wait for all threads to finish
foreach my $thr (threads->list()) {
    $thr->join();
}