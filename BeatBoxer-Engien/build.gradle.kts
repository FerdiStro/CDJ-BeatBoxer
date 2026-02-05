plugins {
  id("java")
  id("application")
}

group = "com.FerdiStro"
version = "1.0-SNAPSHOT"

repositories { mavenCentral() }

dependencies {
  testImplementation(platform("org.junit:junit-bom:5.10.0"))
  testImplementation("org.junit.jupiter:junit-jupiter")
  testRuntimeOnly("org.junit.platform:junit-platform-launcher")

  implementation("org.apache.logging.log4j:log4j-core:2.25.3")

  compileOnly("org.projectlombok:lombok:1.18.34")
  annotationProcessor("org.projectlombok:lombok:1.18.34")

  implementation("org.deepsymmetry:beat-link:8.0.0")
}


application {
  mainClass = "com.FerdiStro.Main"
  applicationDefaultJvmArgs = listOf("-Djava.net.preferIPv4Stack=true", "-Djava.net.preferIPv4Addresses=true")
}

tasks.test { useJUnitPlatform() }
