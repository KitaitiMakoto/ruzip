require "rake/testtask"
require "rubygems/tasks"
require "kar/dsl"

Gem::Tasks.new

Rake::TestTask.new

directory "test/fixtures"
TEST_FIXTURE = "test/fixtures/accessible_epub_3.epub"
file TEST_FIXTURE => "test/fixtures"
download TEST_FIXTURE => "https://github.com/IDPF/epub3-samples/releases/download/20230704/accessible_epub_3.epub"

task test: TEST_FIXTURE

task default: :test
