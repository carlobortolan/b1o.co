# SharedData.pm
package SharedData;
use strict;
use warnings;
use threads::shared;

our %request_counts :shared;
our %visited_urls :shared;
our @shared_queue :shared;

1;