require_relative "helper"

class TestFile < Test::Unit::TestCase
  setup do
    @archive = RuZip::Archive.new(fixture_path("accessible_epub_3.epub"))
  end

  test "retrieve" do
    file = @archive.by_name("META-INF/container.xml")
    assert_instance_of RuZip::File, file
  end

  test "retrieve non-existent" do
    assert_nil @archive.by_name("non-existent")
  end
end
