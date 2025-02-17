Gem::Specification.new do |gem|
  gem.version            = File.read("VERSION").chomp
  gem.date               = File.mtime("VERSION").strftime("%Y-%m-%d")

  gem.name               = "asimov-anthropic-module"
  gem.homepage           = "https://github.com/asimov-modules/asimov-anthropic-module"
  gem.license            = "Unlicense"
  gem.summary            = "ASIMOV Anthropic Module"
  gem.description        = ""
  gem.metadata           = {
    "homepage_uri"      => gem.homepage,
    "bug_tracker_uri"   => "https://github.com/asimov-modules/asimov-anthropic-module/issues",
    "changelog_uri"     => "https://github.com/asimov-modules/asimov-anthropic-module/blob/master/CHANGES.md",
    "documentation_uri" => "https://github.com/asimov-modules/asimov-anthropic-module",
    "source_code_uri"   => "https://github.com/asimov-modules/asimov-anthropic-module",
  }

  gem.author             = "ASIMOV Protocol"
  gem.email              = "support@asimov.so"

  gem.platform           = Gem::Platform::RUBY
  gem.files              = %w(AUTHORS CHANGES.md README.md UNLICENSE VERSION) + Dir.glob("bin/*")
  gem.bindir             = %q(bin)
  gem.executables        = Dir.glob("bin/*").map { |f| File.basename(f) }

  gem.required_ruby_version = ">= 3.2"
  gem.add_development_dependency "rake", ">= 13"
  gem.add_runtime_dependency     "asimov-construct", ">= 25.0.0.dev"
  gem.add_runtime_dependency     "asimov-module", ">= 25.0.0.dev"
  gem.add_runtime_dependency     "mini_mime", "~> 1.1"
  gem.add_runtime_dependency     "omniai", "~> 1.9"
  gem.add_runtime_dependency     "omniai-anthropic", "~> 1.9"
  #gem.add_runtime_dependency     "optparse", ">= 0.3.1" # Ruby 3.2.0
end
