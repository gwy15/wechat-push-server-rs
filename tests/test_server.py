import unittest
import uuid
import requests
import hashlib
import time
import random


class TestCase(unittest.TestCase):
    def setUp(self):
        self.client = requests.session()
        self.token = 'token'

    def url(self, path):
        return 'http://127.0.0.1:8088/api/v2' + path

    def get(self, path, *args, **kws):
        return self.client.get(self.url(path), *args, **kws)

    def post(self, path, *args, **kws):
        return self.client.post(self.url(path), *args, **kws)


class CallbackTest(TestCase):
    @staticmethod
    def sign(token: str, timestamp=None) -> dict:
        if timestamp is None:
            timestamp = int(time.time())
        timestamp = str(timestamp)
        nonce = str(random.randint(1, 1 << 31))
        s = ''.join(sorted([token, timestamp, nonce]))
        signature = hashlib.sha1(s.encode('utf8')).hexdigest()
        return {
            'timestamp': timestamp,
            'nonce': nonce,
            'signature': signature
        }

    def test_callback_negotiate(self):
        echostr = 'should_echo_this'
        params = CallbackTest.sign(self.token)
        params['echostr'] = echostr
        r = self.get('/callback', params=params)
        assert r.status_code == 200
        assert r.text == echostr

    def test_callback_missing_params(self):
        assert self.get('/callback').status_code == 400

    def test_callback_sign_rejected_by_time(self):
        params = CallbackTest.sign(self.token, time.time() - 200)
        params['echostr'] = '123'
        r = self.get('/callback', params=params)
        assert r.status_code == 401
        assert r.json()['errmsg'] == 'Callback signature verification failed'

    def test_callback_sign_rejected_by_sign(self):
        params = CallbackTest.sign(self.token)
        params['signature'] = 'bad_sign'
        params['echostr'] = '123'
        r = self.get('/callback', params=params)
        assert r.status_code == 401


class SceneTest(TestCase):
    def test_scene(self):
        # create
        r = self.post('/scene')
        assert r.status_code == 200
        assert list(r.json().keys()) == ['qr_url', 'scene_id', 'ticket']
        # not found
        scene_id = r.json()['scene_id']
        r = self.get(f'/scene/{scene_id}')
        assert r.status_code == 404
        assert r.json() == {}
        # callback to register
        data = f'''
        <xml>
            <ToUserName><![CDATA[toUser]]></ToUserName>
            <FromUserName><![CDATA[UserOpenID]]></FromUserName>
            <CreateTime>123456789</CreateTime>
            <MsgType><![CDATA[event]]></MsgType>
            <Event><![CDATA[subscribe]]></Event>
            <EventKey><![CDATA[qrscene_{scene_id}]]></EventKey>
            <Ticket><![CDATA[TICKET]]></Ticket>
        </xml>
        '''
        r = self.post('/callback', data, params=CallbackTest.sign(self.token))
        assert r.status_code == 200
        assert r.text == ''
        # now we should get openid
        r = self.get(f'/scene/{scene_id}')
        assert r.status_code == 200
        assert r.json() == {'openID': 'UserOpenID'}


class MessageTest(TestCase):
    @unittest.skip('avoid sending real message')
    def test_post_message(self):
        pass

    def test_post_message_with_illegal_open_id(self):
        form = {
            'title': 'TEST_TITLE',
            'receiver': 'open_id'
        }
        r = self.post('/message', data=form)
        assert r.status_code == 400  # bad request
        assert r.json()['errmsg'] == 'OpenID illegal'

    def test_get_message_not_found(self):
        u = uuid.uuid4()
        r = self.get(f'/message/{u}')
        assert r.status_code == 404
        assert r.json() == {}
