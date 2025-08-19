import { useState } from 'react';
import { useNavigate } from 'react-router';
import { Form, Input, Button, Card, message, Layout } from 'antd';
import { UserOutlined, LockOutlined } from '@ant-design/icons';
import { login, setStoredAuth } from '../utils/auth';

const { Content } = Layout;

export default function Login() {
  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();

  const onFinish = async (values: { username: string; password: string }) => {
    try {
      setLoading(true);
      const response = await login(values);

      // Store authentication data
      setStoredAuth(response.token, {
        id: response.user_id,
        username: response.username,
      });

      message.success('登录成功！');
      navigate('/admin/dashboard');
    } catch (error: any) {
      console.error('Login error:', error);
      if (error.response?.status === 401) {
        message.error('用户名或密码错误');
      } else {
        message.error('登录失败，请检查网络连接');
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <Layout className="min-h-screen bg-gray-50 h-full">
      <Content className="flex items-center justify-center p-4">
        <Card
          title={
            <div className="text-center">
              <h2 className="text-2xl font-bold text-gray-800 mb-2">
                瑜伽馆管理系统
              </h2>
              <p className="text-gray-600">LC PILATES 空中普拉提</p>
            </div>
          }
          className="w-full max-w-md shadow-lg p-4"
          style={{ padding: '8px' }}
        >
          <Form
            name="login"
            onFinish={onFinish}
            autoComplete="off"
            size="large"
            layout="vertical"
          >
            <Form.Item
              label="用户名"
              name="username"
              rules={[{ required: true, message: '请输入用户名!' }]}
            >
              <Input
                prefix={<UserOutlined />}
                placeholder="请输入用户名"
                autoComplete="username"
              />
            </Form.Item>

            <Form.Item
              label="密码"
              name="password"
              rules={[{ required: true, message: '请输入密码!' }]}
            >
              <Input.Password
                prefix={<LockOutlined />}
                placeholder="请输入密码"
                autoComplete="current-password"
              />
            </Form.Item>

            <Form.Item>
              <Button
                type="primary"
                htmlType="submit"
                className="w-full"
                loading={loading}
              >
                登录
              </Button>
            </Form.Item>
          </Form>

          <div className="text-center text-sm text-gray-500 mt-4">
            <p>默认账号：admin</p>
            <p>默认密码：admin123</p>
          </div>
        </Card>
      </Content>
    </Layout>
  );
}