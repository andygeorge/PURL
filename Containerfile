# Use an official Perl image as the base
FROM docker.io/perl:latest
WORKDIR /usr/src/app
COPY PURL.pl .
RUN cpanm HTTP::Tiny
RUN chmod +x PURL.pl
ENTRYPOINT ["./PURL.pl"]

