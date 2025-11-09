require_relative "helper"
require "tmpdir"

class TestWriter < Test::Unit::TestCase
  test "new with path" do
    Dir.mktmpdir do |dir|
      path = File.join(dir, "output.zip")
      writer = RuZip::Writer.new(path)
      assert_kind_of RuZip::Writer, writer
      assert_path_exist path
    end
  end

  test "finish" do
    Dir.mktmpdir do |dir|
      path = File.join(dir, "output.zip")
      writer = RuZip::Writer.new(path)
      archive = writer.finish
      assert_instance_of RuZip::Archive, archive
      assert_raise do
        writer.finish
      end
    end
  end
end
