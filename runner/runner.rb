require "open3"

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
      if File.exist?("aocdeps")
        File.read("aocdeps").lines.map(&:strip)
      else
        []
      end
    end
end

case ARGV[0]
when "run"
  year, day, lang, _ = Dir.pwd.delete_prefix(git_root).delete_prefix("/").split(File::SEPARATOR)

  year = Integer(year)
  day = Integer(day.delete_prefix("d"))

  require File.join(File.dirname(__FILE__), "lang", lang);

  runner = Runner.new(ARGV[1])
  runner.run(load_deps)
when "init"
  year, day, lang, _ = Dir.pwd.delete_prefix(git_root).delete_prefix("/").split(File::SEPARATOR)

  year = Integer(year)
  day = Integer(day.delete_prefix("d"))

  if lang.nil?
    lang = ARGV[1]
    Dir.mkdir(lang)
    Dir.cd(lang)
  end

  require File.join(File.dirname(__FILE__), "lang", lang);

  runner = Runner.new
  runner.init
end
