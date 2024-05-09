# SharedData.pm
package SharedData;
use strict;
use warnings;
use threads::shared;
use MongoDB;
use Config::Simple;
use Data::Dumper;

# Read .env file
my $cfg = new Config::Simple('./../.env');

# Get MongoDB URL from .env
my $mongo_url = $cfg->param('MONGO_URL');
my $mongo_collection = $cfg->param('MONGO_COLLECTION');

# Config
my $client = MongoDB::MongoClient->new(host => $mongo_url);
my $db = $client->get_database($mongo_collection);

# Fetch default values from MongoDB
my $queue_collection = $db->get_collection('queued_sites');
my $visited_urls_collection = $db->get_collection('visited_sites');

print "Fetching default values from MongoDB\n";

our %request_counts :shared;
our %visited_urls :shared = map { $_->{page_url} => 1 } $visited_urls_collection->find->all;
our @shared_queue :shared = map { $_->{page_url} } $queue_collection->find->all;

print "Fetched default values from MongoDB:\n";

1;