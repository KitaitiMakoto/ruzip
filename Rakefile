require "rake/testtask"
require "rubygems/tasks"

Gem::Tasks.new

Rake::TestTask.new

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("ruzip.gemspec")

RbSys::ExtensionTask.new("ruzip", GEMSPEC)

task test: :compile

task default: :test
