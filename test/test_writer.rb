require_relative "helper"
require "tmpdir"
require "archive/zip"

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

  test "add file" do
    Dir.mktmpdir do |dir|
      path = File.join(dir, "output.zip")
      writer = RuZip::Writer.new(path)
      inner_path = "hello.txt"
      writer.start_file(inner_path)
      content = "Hello, World!"
      writer.write content
      writer.finish

      Archive::Zip.open path do |archive|
        file = archive.each.first
        assert_equal inner_path, file.zip_path
        assert_equal content, file.file_data.read(content.length)
      end
    end
  end
end
