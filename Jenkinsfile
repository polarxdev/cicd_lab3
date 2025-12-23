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