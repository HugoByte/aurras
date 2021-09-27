import React, { useState } from 'react';
import Modal from 'react-modal';
import config from "./config";
import firebase from "firebase";

const customStyles = {
    content: {
        width: '50%',
        top: '50%',
        left: '50%',
        right: 'auto',
        bottom: 'auto',
        marginRight: '-50%',
        transform: 'translate(-50%, -50%)',
    },
};

Modal.setAppElement('#root');

function ModalApp(props) {

    const messaging = firebase.messaging();
    const [modalIsOpen, setIsOpen] = React.useState(false);
    const [topics, setTopics] = useState([]);
    const [selected, setSelected] = useState("");
    const [token, setToken] = useState("");
    const [notification, setNotification] = React.useState({
        visible: false,
        title: "",
        message: "",
        variant: ""
    });
    const [busy, setBusy] = useState(false);
    messaging.getToken().then(token => setToken(token))
    .catch((err) => {
        console.log(err);
    });
    function openModal() {
        setIsOpen(true);
    }

    function selectTopic(event) {
        setSelected(event.target.value);
    }

    function afterOpenModal() {
        fetch(`${config.api}default/balance-notification-registration`)
            .then(response => response.json())
            .then(data => {
                if (data[0]) setSelected(data[0].doc.trigger)
                setTopics(data);
            });

    }

    function closeModal() {
        setIsOpen(false);
    }

    function register({ address, topic, token }) {
        setBusy(true);
        var requestOptions = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ address, topic, token }),
        };
        fetch(`${config.api}default/balance-notification-registration`, requestOptions)
            .then(response => response.json())
            .then(data => {
                setNotification({
                    visible: true,
                    title: "Success",
                    message: "Registered!",
                    variant: "positive"
                })
            })
            .catch((error) => {
                setNotification({
                    visible: true,
                    title: "Failed",
                    message: "Not Registered!",
                    variant: "negative"
                })
            }).finally(() => {
                setBusy(false);
                setTimeout(() => {
                    setNotification({
                        visible: false,
                        title: "",
                        message: "",
                        variant: ""
                    })
                }, 5000);
            });
    }
    return (
        <div>
            <button onClick={openModal} className="ui button">Register Balance Notification</button>
            <Modal
                isOpen={modalIsOpen}
                onAfterOpen={afterOpenModal}
                onRequestClose={closeModal}
                style={customStyles}
                contentLabel="Register Balance Notification"
            >
                <div className="ui grid">
                    <div className="ten column row">
                        <div className="right floated column">
                            <button className="circular ui icon button" onClick={closeModal}>
                                <i className="icon close"></i>
                            </button>
                        </div>
                    </div>
                </div>
                {
                    notification.visible && <div className={"ui message ".concat(notification.variant)}>
                        <i className="close icon"></i>
                        <div className="header">
                            {notification.title}
                        </div>
                        <p>{notification.message}</p>
                    </div>
                }

                <div>
                    <div className="label">
                        Event Source
                    </div>
                    <div className="ui fluid">
                        <select className="ui dropdown w-full pt pb" onChange={selectTopic} value={selected}>
                            {
                                topics.map(function (item) {
                                    return <option key={item.doc.trigger} value={item.doc.trigger}>{item.doc.name}</option>
                                })
                            }
                        </select>
                    </div>
                </div>
                <div>
                    <div className="label">
                        Address
                    </div>
                    <div className="ui fluid disabled input pt pb">
                        <input value={props.address} readOnly />
                    </div>
                </div>
                <div>
                    <div className="label">
                        Push Notification Token
                    </div>
                    <div className="ui fluid disabled input pt pb">
                        <input value={token} readOnly />
                    </div>
                </div>
                <div className="ui grid">
                    <div className="four column centered row">
                        <button onClick={() => register({ address: props.address, topic: selected, token })} className={"ui primary button ".concat(busy ? "disabled loading" : "")}>register</button>
                        <button onClick={closeModal} className="ui button">close</button>
                    </div>
                </div>


            </Modal>
        </div>
    );
}

export default ModalApp