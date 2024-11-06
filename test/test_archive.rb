require_relative "helper"

class TestArchive < Test::Unit::TestCase
  test "new with path" do
    archive = RuZip::Archive.new(fixture_path("book.epub"))
    assert_kind_of RuZip::Archive, archive
  end

  test "new with pathname" do
    archive = RuZip::Archive.new(Pathname(fixture_path("book.epub")))
    assert_kind_of RuZip::Archive, archive
  end

  test "new with file" do
    archive = RuZip::Archive.new(File.open(fixture_path("book.epub")))
    assert_kind_of RuZip::Archive, archive
  end

  test "new with closed file" do
    io = nil
    File.open fixture_path("book.epub") do |file|
      io = file
    end
    assert_raise_kind_of IOError do
      RuZip::Archive.new(io)
    end
  end

  test "new with string io" do
    pend

    require "stringio"
    io = StringIO.new("EPUB file")
    archive = RuZip::Archive.new(io)
    assert_kind_of RuZip::Archive, archive
  end

  test "new with unsupported type" do
    assert_raise_kind_of TypeError do
      RuZip::Archive.new(:symbol_object)
    end
  end

  test "len" do
    archive = RuZip::Archive::new(fixture_path("book.epub"))
    assert_equal 20, archive.length
  end
end
