import { useState, useEffect } from 'react';
import { 
  Table, 
  Card, 
  Button, 
  Space, 
  Modal, 
  Form, 
  Input, 
  Switch,
  message,
  Popconfirm,
  Tag,
  Typography
} from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, UserOutlined } from '@ant-design/icons';
import { api } from '../../utils/auth';

const { Text } = Typography;

interface AdminUser {
  id: number;
  username: string;
  is_active?: boolean;
  created_at?: string;
  updated_at?: string;
}

export default function AdminUsers() {
  const [adminUsers, setAdminUsers] = useState<AdminUser[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingUser, setEditingUser] = useState<AdminUser | null>(null);
  const [form] = Form.useForm();

  const fetchAdminUsers = async () => {
    try {
      setLoading(true);
      const response = await api.get('/admin/admin-users');
      setAdminUsers(response.data);
    } catch (error) {
      message.error('获取管理员列表失败');
      console.error('Error fetching admin users:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchAdminUsers();
  }, []);

  const handleAdd = () => {
    setEditingUser(null);
    form.resetFields();
    setModalVisible(true);
  };

  const handleEdit = (user: AdminUser) => {
    setEditingUser(user);
    form.setFieldsValue({
      username: user.username,
      is_active: user.is_active
    });
    setModalVisible(true);
  };

  const handleDelete = async (id: number, username: string) => {
    if (id === 1) {
      message.warning('默认管理员用户不能删除');
      return;
    }

    try {
      await api.delete(`/admin/admin-users/${id}`);
      message.success(`管理员用户"${username}"删除成功`);
      fetchAdminUsers();
    } catch (error: any) {
      if (error.response?.data?.error) {
        message.error(error.response.data.error);
      } else {
        message.error('删除管理员用户失败');
      }
      console.error('Error deleting admin user:', error);
    }
  };

  const handleSubmit = async (values: any) => {
    try {
      if (editingUser) {
        // Update admin user
        await api.put(`/admin/admin-users/${editingUser.id}`, values);
        message.success('管理员用户更新成功');
      } else {
        // Create new admin user
        await api.post('/admin/admin-users', values);
        message.success('管理员用户创建成功');
      }
      setModalVisible(false);
      fetchAdminUsers();
    } catch (error: any) {
      if (error.response?.status === 409) {
        message.error('用户名已存在');
      } else if (error.response?.data?.error) {
        message.error(error.response.data.error);
      } else {
        message.error(editingUser ? '更新管理员用户失败' : '创建管理员用户失败');
      }
      console.error('Error saving admin user:', error);
    }
  };

  const columns = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
    },
    {
      title: '用户名',
      dataIndex: 'username',
      key: 'username',
      render: (username: string, record: AdminUser) => (
        <Space>
          <UserOutlined />
          <Text strong={record.id === 1}>{username}</Text>
          {record.id === 1 && <Tag color="gold">默认管理员</Tag>}
        </Space>
      ),
    },
    {
      title: '状态',
      dataIndex: 'is_active',
      key: 'is_active',
      render: (is_active: boolean) => (
        <Tag color={is_active ? 'green' : 'red'}>
          {is_active ? '激活' : '停用'}
        </Tag>
      ),
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      render: (date: string) => date ? new Date(date).toLocaleString() : '-',
    },
    {
      title: '更新时间',
      dataIndex: 'updated_at',
      key: 'updated_at',
      render: (date: string) => date ? new Date(date).toLocaleString() : '-',
    },
    {
      title: '操作',
      key: 'actions',
      render: (_, record: AdminUser) => (
        <Space>
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title={record.id === 1 ? "默认管理员用户不能删除" : `确定要删除管理员"${record.username}"吗？`}
            onConfirm={() => handleDelete(record.id, record.username)}
            disabled={record.id === 1}
          >
            <Button
              type="link"
              danger
              icon={<DeleteOutlined />}
              disabled={record.id === 1}
            >
              删除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <>
      <Card
        title="管理员用户管理"
        extra={
          <Button
            type="primary"
            icon={<PlusOutlined />}
            onClick={handleAdd}
          >
            添加管理员
          </Button>
        }
      >
        <Table
          columns={columns}
          dataSource={adminUsers}
          rowKey="id"
          loading={loading}
          pagination={{
            showSizeChanger: true,
            showTotal: (total) => `共 ${total} 个管理员`,
          }}
        />

        <Modal
          title={editingUser ? '编辑管理员' : '添加管理员'}
          open={modalVisible}
          onCancel={() => setModalVisible(false)}
          footer={null}
          destroyOnClose
        >
          <Form
            form={form}
            layout="vertical"
            onFinish={handleSubmit}
          >
            <Form.Item
              name="username"
              label="用户名"
              rules={[
                { required: true, message: '请输入用户名' },
                { min: 3, message: '用户名至少需要3个字符' },
                { max: 50, message: '用户名不能超过50个字符' },
                { pattern: /^[a-zA-Z0-9_]+$/, message: '用户名只能包含字母、数字和下划线' }
              ]}
            >
              <Input placeholder="请输入用户名" />
            </Form.Item>

            <Form.Item
              name="password"
              label="密码"
              rules={[
                { required: !editingUser, message: '请输入密码' },
                { min: 6, message: '密码至少需要6个字符' }
              ]}
            >
              <Input.Password placeholder={editingUser ? "留空以保持当前密码" : "请输入密码"} />
            </Form.Item>

            {editingUser && (
              <Form.Item
                name="is_active"
                label="状态"
                valuePropName="checked"
                extra={editingUser?.id === 1 ? "默认管理员用户不能停用" : ""}
              >
                <Switch 
                  checkedChildren="激活" 
                  unCheckedChildren="停用"
                  disabled={editingUser?.id === 1}
                />
              </Form.Item>
            )}

            <Form.Item>
              <Space>
                <Button type="primary" htmlType="submit">
                  {editingUser ? '更新' : '创建'}
                </Button>
                <Button onClick={() => setModalVisible(false)}>
                  取消
                </Button>
              </Space>
            </Form.Item>
          </Form>
        </Modal>
      </Card>
    </>
  );
}