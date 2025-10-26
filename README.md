# RuZip

Library to support the reading and writing of zip files. A wrapper of Rust's [zip](https://github.com/zip-rs/zip2?tab=readme-ov-file) crate.

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add ruzip

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install ruzip

## Usage

    require "ruzip"
    
    archive = RuZip::Archive.new("path/to/archive.zip")
    archive.length #=> Integer, the number of files the archive holds
    
    archive.length.times do |i|
      file = archive.by_index(i) #=> RuZip::File
      file.name # => String
      file.size # => Integer, the size of decompressed file
      file.last_modified # => Time
      content = file.read # => ASCII-8BIT String (BINARY)
      content.force_encoding("UTF-8") # If it's text, it is in UTF-8.
    end
    
    archive.by_name("path/in/zip/archive") # => RuZip::File

## Development

After checking out the repo, run `bundle install` to install dependencies. Then, run `bundle exec rake test` to run the tests. You can also run `bundle exec rake console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `ext/Cargo.toml`, run `cargo update --manifest-path=ext/Cargo.toml`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitLab at https://gitlab.com/KitaitiMakoto/ruzip. This project is intended to be a safe, welcoming space for collaboration.
