import QtQuick 2.15
import QtQuick.Controls 2.15

ApplicationWindow {
    visible: true
    width: 600
    height: 700
    title: "Поле чудес"
    id: root

    property var sectorColors: ["#FF4D4D", "#4D79FF", "#FF00FF", "#FF8000", "#00FF00"]
    property var sectorNames: ["Результативный ход", "Музыкальная пауза", "Сектор Плюс", "Сектор Шанс", "Сектор Приз"]
    property string displayText: "Крутите колесо!"

    NumberAnimation {
        id: spinAnimation
        target: wheelContainer
        property: "rotation"
        duration: 3500
        easing.type: Easing.OutCubic
        onStopped: {
            displayText = rustWheel.sector_text
        }
    }

    Column {
        anchors.centerIn: parent
        spacing: 20

        Canvas {
            width: 24
            height: 30
            anchors.horizontalCenter: parent.horizontalCenter
            z: 2

            onPaint: {
                var ctx = getContext("2d");
                ctx.reset();
                ctx.fillStyle = "red";
                ctx.beginPath();
                ctx.moveTo(0, 0);
                ctx.lineTo(width, 0);
                ctx.lineTo(width / 2, height);
                ctx.closePath();
                ctx.fill();
            }
        }

        Rectangle {
            width: 400
            height: 400
            radius: 200
            border.color: "black"
            border.width: 4
            clip: false

            Item {
                id: wheelContainer
                anchors.fill: parent
                rotation: 0

                Repeater {
                    model: 5
                    delegate: Canvas {
                        width: 400
                        height: 400
                        rotation: index * 72

                        onPaint: {
                            var ctx = getContext("2d");
                            ctx.reset();
                            ctx.beginPath();
                            ctx.moveTo(200, 200);
                            ctx.arc(200, 200, 200, -Math.PI / 2, -Math.PI / 2 + (72 * Math.PI / 180), false);
                            ctx.lineTo(200, 200);
                            ctx.fillStyle = sectorColors[index];
                            ctx.fill();
                            ctx.lineWidth = 2;
                            ctx.strokeStyle = "black";
                            ctx.stroke();
                        }
                    }
                }

                Repeater {
                    model: 5
                    delegate: Item {
                        width: 400
                        height: 400
                        rotation: index * 72 + 36

                        Text {
                            text: sectorNames[index]
                            y: 30
                            anchors.horizontalCenter: parent.horizontalCenter
                            font.bold: true
                            font.pixelSize: 16
                            color: "black"
                        }
                    }
                }
            }
        }

        Button {
            text: "Крутить"
            anchors.horizontalCenter: parent.horizontalCenter
            enabled: !spinAnimation.running

            onClicked: {
                displayText = "Барабан крутится..."
                rustWheel.spin_to_next_scripted()
                spinAnimation.from = wheelContainer.rotation
                spinAnimation.to = rustWheel.rotation_angle
                spinAnimation.start()
            }
        }

        Rectangle {
            width: parent.width * 0.8
            height: 70
            color: "#2b2b3b"
            radius: 10
            border.color: "#ffcc00"
            border.width: 2
            anchors.horizontalCenter: parent.horizontalCenter

            Text {
                anchors.centerIn: parent
                width: parent.width - 20
                text: displayText
                color: "#ffffff"
                font.pixelSize: 18
                font.bold: true
                wrapMode: Text.WordWrap
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }
    }
}