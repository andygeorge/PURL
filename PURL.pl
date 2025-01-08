#!/usr/bin/perl
use strict;
use warnings;
use HTTP::Tiny;

my $url = shift or die "Usage: $0 <URL>\n";
my $http = HTTP::Tiny->new;
my $response = $http->get($url);

if ($response->{success}) {
    print "Response:\n", $response->{content}, "\n";
} else {
    die "HTTP GET failed: $response->{status} $response->{reason}\n";
}
