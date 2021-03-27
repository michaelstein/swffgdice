from conans import ConanFile, CMake

# cargo install --force cbindgen

class SwFfgDiceConan(ConanFile):
	name = "swffgdice"
	version = "0.1"
	license = "MIT license"
	url = "https://github.com/michaelstein/swffgdice"
	description = "SW FFG Dice Roller"
	settings = "os", "compiler", "build_type", "arch"
	options = {
		"shared": [True, False]
	}
	default_options = {
		"shared": False
	}
	exports_sources = "src/*", "Cargo.toml", "Cargo.lock", "cbindgen.toml", "LICENSE"

	def build(self):
		cmd = ["cargo", "build", "--lib"]
		cmd.append("--crate-type=cdylib" if self.options.shared "--crate-type=staticlib")

		build_type = self.settings.get_safe("build_type", default="Release")
		if build_type == "Release":
			cmd.append("--release")
		else:
			cmd.append("--debug")
			cmd.append("-C opt-level=0")

		env_build = RunEnvironment(self)
		with tools.environment_append(env_build.vars):
			self.run(" ".join(cmd))
			self.run("cbindgen --config cbindgen.toml --crate swffgdice --output include/swffgdice/lib.h")

	def package(self):
		build_type = self.settings.get_safe("build_type", default="Release").lower()
		self.copy("*.h", dst="include", src="include")
		self.copy("*.lib", dst="lib", src=os.path.join("target", build_type))
		self.copy("*.dll", dst="bin", src=os.path.join("target", build_type))
		self.copy("*.dylib*", dst="lib", src=os.path.join("target", build_type))
		self.copy("*.so", dst="lib", src=os.path.join("target", build_type))
		self.copy("*.a", dst="lib", src=os.path.join("target", build_type))

	def package_info(self):
		self.cpp_info.libs = ["swffgdice"]
