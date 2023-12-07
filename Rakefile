require "bundler/gem_tasks"
require "rake/testtask"

Rake::TestTask.new(:test) do |t|
  t.test_files = FileList["test/**/*_test.rb"]
end

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("ruzip.gemspec")

RbSys::ExtensionTask.new("ruzip", GEMSPEC) do |ext|
  ext.lib_dir = "lib/ruzip"
end

task default: %i[compile test]
