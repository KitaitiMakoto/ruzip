require "rake/clean"
require "rake/testtask"
require "rubygems/ext"
require "rubygems/tasks"
require "kar/dsl"

cargo "ruzip"

Gem::Tasks.new
task build: "cargo:validate"

Rake::TestTask.new

directory "test/fixtures"
TEST_FIXTURE = "test/fixtures/accessible_epub_3.epub"
file TEST_FIXTURE => "test/fixtures"
download TEST_FIXTURE => "https://github.com/IDPF/epub3-samples/releases/download/20230704/accessible_epub_3.epub"

task test: [:cargo, TEST_FIXTURE]

task default: :test
