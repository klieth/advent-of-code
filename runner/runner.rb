require "open3"
require "yaml"

def git_root
  @git_root ||= begin
    out, status = Open3.capture2("git", "rev-parse", "--show-toplevel")
    if status.exitstatus != 0
      puts "Failed to get git root: process exited with code #{status.exitstatus}"
      exit 1
    end
    out.strip
  end
end

def template_dir(lang)
  File.join(git_root, "templates", lang)
end

def lib_dir(lang, lib)
  File.join(git_root, "lib", lang, lib)
end

def load_deps
  @deps ||=
    begin
      if File.exist?("aocdeps.yaml")
        YAML.load(File.read("aocdeps.yaml"))
      else
        {}
      end
    end
end

def parse_directory
  year, day, lang, _ = Dir.pwd.delete_prefix(git_root).delete_prefix("/").split(File::SEPARATOR)

  year = Integer(year)
  day = Integer(day.delete_prefix("d"))

  [year, day, lang]
end

# === OPS ===

def init
  year, day, lang = parse_directory

  if lang.nil?
    lang = ARGV[1]
    Dir.mkdir(lang)
    Dir.cd(lang)
  end

  require File.join(File.dirname(__FILE__), "lang", lang);

  runner = Runner.new
  runner.init

  runner
end

def build
  year, day, lang = parse_directory

  require File.join(File.dirname(__FILE__), "lang", lang);

  runner = Runner.new
  if !runner.build(load_deps)
    throw "build failed"
  end

  runner
end

def run(file)
  runner = build

  runner.run(file)

  runner
end

OPS = ["init", "build", "run"]

if OPS.include?(ARGV[0])
  m = method(ARGV[0])
  m.call(*ARGV[1..m.arity])
else
  puts "unrecognized op #{ARGV[0]}"
  exit 1
end
