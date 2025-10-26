require_relative "helper"

class TestFile < Test::Unit::TestCase
  setup do
    @archive = RuZip::Archive.new(fixture_path("accessible_epub_3.epub"))
  end

  test "retrieve" do
    file = @archive.by_name("META-INF/container.xml")
    assert_instance_of RuZip::File, file
    assert_equal <<XML.gsub(/\r\n/, "\n"), file.read.force_encoding('UTF-8').gsub(/\r\n/, "\n")
<?xml version="1.0" encoding="utf-8" standalone="no"?>
<container xmlns="urn:oasis:names:tc:opendocument:xmlns:container" version="1.0">
	<rootfiles>
		<rootfile full-path="EPUB/package.opf" media-type="application/oebps-package+xml"/>
	</rootfiles>
</container>
XML
  end

  test "retrieve non-existent" do
    assert_nil @archive.by_name("non-existent")
  end
end
