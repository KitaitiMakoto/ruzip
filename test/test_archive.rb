require_relative "helper"

class TestArchive < Test::Unit::TestCase
  def setup
    @fixture = fixture_path("accessible_epub_3.epub")
  end

  test "new with path" do
    archive = RuZip::Archive.new(@fixture)
    assert_kind_of RuZip::Archive, archive
  end

  test "new with pathname" do
    archive = RuZip::Archive.new(Pathname(@fixture))
    assert_kind_of RuZip::Archive, archive
  end

  test "new with file" do
    archive = RuZip::Archive.new(File.open(@fixture))
    assert_kind_of RuZip::Archive, archive
  end

  test "new with closed file" do
    io = nil
    File.open @fixture do |file|
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
    archive = RuZip::Archive::new(@fixture)
    assert_equal 38, archive.length
  end
end
