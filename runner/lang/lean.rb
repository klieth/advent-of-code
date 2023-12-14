require "fileutils"

class Runner
  def init
    FileUtils.cp(Dir.glob(File.join(template_dir("lean"), "*")), ".")
  end
end
