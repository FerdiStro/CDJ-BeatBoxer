import sun.jvmstat.monitor.MonitoredVmUtil.mainClass

plugins {
  id("java")
  id("application")
}

group = "com.FerdiStro"
version = "1.0-SNAPSHOT"

repositories {


  mavenCentral()
  maven("https://clojars.org/repo")
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


//tasks.register<JavaExec>("run") {
//  group = "application"
//  description = "Run Drum machine for Tests"
//
//}
//tasks.register('runMinimal', JavaExec) {
//  group = 'application'
//  description = 'Runs MinimalRunner.Main'
//  classpath = sourceSets.main.runtimeClasspath
//  mainClass = 'MinimalRunner.Main'
//}

application {
  mainClass = "com.FerdiStro.Main"
  applicationDefaultJvmArgs = listOf("-Djava.net.preferIPv4Stack=true", "-Djava.net.preferIPv4Addresses=true")
}

tasks.test { useJUnitPlatform() }
