require_relative "helper"

class TestRuZip < Test::Unit::TestCase
  test "version" do
    assert_equal "0.1.0", Gem.loaded_specs["ruzip"].version.to_s
  end
end
