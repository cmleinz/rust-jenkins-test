pipeline {
  
  agent any
  
  stages {

    stage("lint") {
      steps {
        sh "cargo clippy"
      }
    }

    stage("build") {
      steps {
        sh "cargo b"
      }
    }

    stage("test") {
      steps {
        sh "cargo t"
      }
    }
  }
}
