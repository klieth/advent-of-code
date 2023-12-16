require "fileutils"

class Runner
  def init
    throw "already initialized" if File.exist?("Main.lean")
    FileUtils.cp(Dir.glob(File.join(template_dir("lean"), "*")), ".")
  end
end
