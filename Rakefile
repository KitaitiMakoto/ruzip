require "rake/clean"
require "rake/testtask"
require "rubygems/ext"
require "rubygems/tasks"
require "kar/dsl"
require "rdoc/task"

cargo "ruzip"

Gem::Tasks.new
task build: "cargo:check"

TEST_FIXTURE = "test/fixtures/accessible_epub_3.epub"
download TEST_FIXTURE => "https://github.com/IDPF/epub3-samples/releases/download/20230704/accessible_epub_3.epub"

Rake::TestTask.new test: [:cargo, TEST_FIXTURE]

RDoc::Task.new

task default: :test
