plugins {
  id("java")
  id("application")
  id("org.beryx.jlink") version "3.2.1"
}

group = "com.FerdiStro"

version = "1.0-SNAPSHOT"

repositories {
  mavenCentral()
  maven("https://clojars.org/repo")
}

tasks.register<Zip>("bundleApp") {
  description = "Zip JVM and Engien"
  group = JavaBasePlugin.BUILD_TASK_NAME

  dependsOn("installDist", "createJre")

  archiveFileName.set("beat-boxer_engien.zip")
  destinationDirectory.set(layout.buildDirectory.dir("bundle"))

  from(layout.buildDirectory.dir("custom-jre")) { into("jre") }

  from(layout.buildDirectory.dir("install/${project.name}")) { into("app") }
}

tasks.register<Exec>("createJre") {
  description = "Create Custom mini-JVM for MINIM (Sound-library)"
  group = JavaBasePlugin.BUILD_TASK_NAME

  val jreDir = layout.buildDirectory.dir("custom-jre").get().asFile
  outputs.dir(jreDir)

  doFirst { jreDir.deleteRecursively() }

  val javaHome = System.getProperty("java.home")

  commandLine(
      "$javaHome/bin/jlink",
      "--add-modules",
      "java.base,java.desktop,java.logging,jdk.unsupported",
      "--strip-debug",
      "--no-man-pages",
      "--no-header-files",
      "--compress=2",
      "--output",
      jreDir.absolutePath,
  )
}

dependencies {
  testImplementation(platform("org.junit:junit-bom:5.10.0"))
  testImplementation("org.junit.jupiter:junit-jupiter")
  testRuntimeOnly("org.junit.platform:junit-platform-launcher")

  implementation("org.apache.logging.log4j:log4j-core:2.25.3")

  compileOnly("org.projectlombok:lombok:1.18.34")
  annotationProcessor("org.projectlombok:lombok:1.18.34")

  implementation("org.deepsymmetry:beat-link:8.0.0")

  implementation("ddf.minim:ddf.minim:2.2.0")
}

application {
  mainClass = "com.FerdiStro.Main"
  applicationDefaultJvmArgs =
      listOf("-Djava.net.preferIPv4Stack=true", "-Djava.net.preferIPv4Addresses=true")
}

tasks.test { useJUnitPlatform() }
