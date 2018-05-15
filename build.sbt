ThisBuild / scalaVersion := "2.12.6"

lazy val hello = (project in file("."))
  .settings(
    name := "Santorini",
    libraryDependencies += "org.scalatest" %% "scalatest" % "3.0.5" % Test,
  )
