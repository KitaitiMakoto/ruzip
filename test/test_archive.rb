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

  test "read_by_name" do
    archive = RuZip::Archive::new(fixture_path("book.epub"))
    assert_equal <<EOS, archive.read_by_name("META-INF/container.xml").force_encoding("UTF-8")
<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="OPS/ルートファイル.opf" media-type="application/oebps-package+xml" />
    <rootfile full-path="nested/dir/content.opf" media-type="application/oebps-package+xml" />
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml" />
  </rootfiles>
</container>
EOS
  end
end
