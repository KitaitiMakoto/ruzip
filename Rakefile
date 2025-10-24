require "rake/clean"
require "rake/testtask"
require "rubygems/ext"
require "rubygems/tasks"
require "kar/dsl"

Gem::Tasks.new

GEMSPEC = Gem::Specification.load("ruzip.gemspec")
MANIFEST = GEMSPEC.extensions[0]
SRC = FileList["ext/**/*.rs"]
DL_NAME = "#{GEMSPEC.name}.#{RbConfig::CONFIG["DLEXT"]}"
DL_PATH = File.join("lib", DL_NAME)
file DL_PATH => SRC + [MANIFEST] do
  results = Rake.verbose == true ? $stdout : []
  begin
    Gem::Ext::CargoBuilder.new.build MANIFEST, ".", results, [], "lib", File.expand_path("ext")
  rescue => error
    $stderr.puts results unless Rake.verbose == true
    fail
  end
end
CLEAN.include DL_NAME
CLEAN.include DL_PATH

Rake::TestTask.new

directory "test/fixtures"
TEST_FIXTURE = "test/fixtures/accessible_epub_3.epub"
file TEST_FIXTURE => "test/fixtures"
download TEST_FIXTURE => "https://github.com/IDPF/epub3-samples/releases/download/20230704/accessible_epub_3.epub"

task test: [DL_PATH, TEST_FIXTURE]

task default: :test
