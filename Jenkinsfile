pipeline {
    agent any

    environment {
        BRANCH_NAME = "${env.BRANCH_NAME}"
    }

    stages {
        stage('Checkout') {
            steps {
                echo "Сборка в ветке: ${env.BRANCH_NAME}"
            }
        }

        stage('Build') {
            steps {
                sh 'cargo build --release'
                echo 'Сборка завершена'
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test'
                echo 'Тесты пройдены'
            }
        }

        stage('Artifact') {
            steps {
                sh 'cp target/release/jenkins-rust-demo ./rust-artifact-${BRANCH_NAME}'
                archiveArtifacts artifacts: 'rust-artifact-*', fingerprint: true
            }
        }

        stage('Branch Specific') {
            when {
                expression { env.BRANCH_NAME == 'main' }
            }
            steps {
                echo 'Это продакшен-ветка! Готово к деплою.'
            }
        }
    }

    post {
        success {
            echo 'Билд успешный в ветке ${env.BRANCH_NAME}'
        }
        failure {
            echo 'Билд провален!'
        }
    }
}