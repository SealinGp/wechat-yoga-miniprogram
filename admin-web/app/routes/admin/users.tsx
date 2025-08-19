import { useState, useEffect } from 'react';
import { 
  Table, 
  Card, 
  Button, 
  Space, 
  Modal, 
  Form, 
  Input, 
  message,
  Popconfirm,
  Typography,
  Avatar,
  Tag
} from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, UserOutlined } from '@ant-design/icons';
import { api } from '../../utils/auth';

const { Text } = Typography;

interface User {
  id: number;
  open_id: string;
  nick_name?: string;
  avatar_url?: string;
  phone?: string;
  created_at?: string;
  updated_at?: string;
}

export default function Users() {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingUser, setEditingUser] = useState<User | null>(null);
  const [form] = Form.useForm();

  const fetchUsers = async () => {
    try {
      setLoading(true);
      const response = await api.get('/admin/users');
      setUsers(response.data);
    } catch (error) {
      message.error('获取用户列表失败');
      console.error('Error fetching users:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchUsers();
  }, []);

  const handleAdd = () => {
    setEditingUser(null);
    form.resetFields();
    setModalVisible(true);
  };

  const handleEdit = (user: User) => {
    setEditingUser(user);
    form.setFieldsValue({
      open_id: user.open_id,
      nick_name: user.nick_name,
      avatar_url: user.avatar_url,
      phone: user.phone
    });
    setModalVisible(true);
  };

  const handleDelete = async (id: number, nick_name?: string) => {
    try {
      await api.delete(`/admin/users/${id}`);
      message.success(`用户"${nick_name || '未知'}"删除成功`);
      fetchUsers();
    } catch (error: any) {
      message.error('删除用户失败');
      console.error('Error deleting user:', error);
    }
  };

  const handleSubmit = async (values: any) => {
    try {
      if (editingUser) {
        // Update user - exclude open_id from update payload
        const { open_id, ...updateValues } = values;
        await api.put(`/admin/users/${editingUser.id}`, updateValues);
        message.success('用户更新成功');
      } else {
        // Create new user
        await api.post('/admin/users', values);
        message.success('用户创建成功');
      }
      setModalVisible(false);
      fetchUsers();
    } catch (error: any) {
      message.error(editingUser ? '更新用户失败' : '创建用户失败');
      console.error('Error saving user:', error);
    }
  };

  const columns = [
    {
      title: '头像',
      dataIndex: 'avatar_url',
      key: 'avatar_url',
      width: 80,
      render: (avatar_url: string) => (
        avatar_url ? (
          <Avatar src={avatar_url} size={40} />
        ) : (
          <Avatar icon={<UserOutlined />} size={40} />
        )
      ),
    },
    {
      title: '昵称',
      dataIndex: 'nick_name',
      key: 'nick_name',
      render: (nick_name: string) => nick_name || <Text type="secondary">未设置</Text>,
    },
    {
      title: '微信OpenID',
      dataIndex: 'open_id',
      key: 'open_id',
      render: (open_id: string) => (
        <Text code style={{ fontSize: '12px' }}>
          {open_id.length > 20 ? `${open_id.substring(0, 20)}...` : open_id}
        </Text>
      ),
    },
    {
      title: '手机号',
      dataIndex: 'phone',
      key: 'phone',
      render: (phone: string) => phone || <Text type="secondary">未提供</Text>,
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      render: (created_at: string) => {
        if (!created_at) return '-';
        return new Date(created_at).toLocaleDateString();
      },
    },
    {
      title: '操作',
      key: 'actions',
      render: (_, record: User) => (
        <Space>
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title={`确定要删除用户"${record.nick_name || '该用户'}"吗？`}
            onConfirm={() => handleDelete(record.id, record.nick_name)}
          >
            <Button
              type="link"
              danger
              icon={<DeleteOutlined />}
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
        title="用户管理"
        extra={
          <Button
            type="primary"
            icon={<PlusOutlined />}
            onClick={handleAdd}
          >
            添加用户
          </Button>
        }
      >
        <Table
          columns={columns}
          dataSource={users}
          rowKey="id"
          loading={loading}
          pagination={{
            showSizeChanger: true,
            showTotal: (total) => `共 ${total} 个用户`,
          }}
        />

        <Modal
          title={editingUser ? '编辑用户' : '添加用户'}
          open={modalVisible}
          onCancel={() => setModalVisible(false)}
          footer={null}
          destroyOnClose
          width={600}
        >
          <Form
            form={form}
            layout="vertical"
            onFinish={handleSubmit}
          >
            <Form.Item
              name="open_id"
              label="微信 Open ID"
              rules={[
                { required: true, message: '请输入微信 Open ID' },
                { min: 10, message: 'Open ID 至少需要10个字符' }
              ]}
            >
              <Input 
                placeholder="请输入微信 Open ID" 
                disabled={!!editingUser}
                style={editingUser ? { backgroundColor: '#f5f5f5' } : {}}
              />
            </Form.Item>

            {editingUser && (
              <div style={{ marginBottom: 16 }}>
                <Text type="secondary" style={{ fontSize: '12px' }}>
                  注意：现有用户的 Open ID 不能修改
                </Text>
              </div>
            )}

            <Form.Item
              name="nick_name"
              label="昵称"
              rules={[
                { max: 100, message: '昵称不能超过100个字符' }
              ]}
            >
              <Input placeholder="请输入昵称" />
            </Form.Item>

            <Form.Item
              name="avatar_url"
              label="头像链接"
              rules={[
                { type: 'url', message: '请输入有效的URL' }
              ]}
            >
              <Input placeholder="请输入头像链接" />
            </Form.Item>

            <Form.Item
              name="phone"
              label="手机号"
              rules={[
                { pattern: /^1[3-9]\d{9}$/, message: '请输入有效的中国手机号' }
              ]}
            >
              <Input placeholder="请输入手机号" />
            </Form.Item>

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