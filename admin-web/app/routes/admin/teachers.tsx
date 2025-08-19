import { useState, useEffect } from 'react';
import {
  Table,
  Card,
  Button,
  Space,
  Modal,
  Form,
  Input,
  InputNumber,
  Switch,
  message,
  Popconfirm,
  Avatar,
  Tag,
  Rate,
  Upload
} from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, UserOutlined, UploadOutlined } from '@ant-design/icons';
import { api, mockApi } from '../../utils/auth';

const apiClient = api;

const { TextArea } = Input;

interface Teacher {
  id: number;
  name: string;
  description?: string;
  avatar_url?: string;
  bio?: string;
  certifications?: string[];
  specialties?: string[];
  experience_years?: number;
  average_rating?: number;
  total_ratings?: number;
  is_active?: boolean;
  created_at?: string;
}

export default function Teachers() {
  const [teachers, setTeachers] = useState<Teacher[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingTeacher, setEditingTeacher] = useState<Teacher | null>(null);
  const [uploading, setUploading] = useState(false);
  const [form] = Form.useForm();

  const fetchTeachers = async () => {
    try {
      setLoading(true);
      const response = await apiClient.get('/admin/teachers');
      setTeachers(response.data);
    } catch (error) {
      message.error('获取教师列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTeachers();
  }, []);

  const handleCreate = () => {
    setEditingTeacher(null);
    form.resetFields();
    form.setFieldsValue({
      experience_years: 0,
      is_active: true,
    });
    setModalVisible(true);
  };

  const handleEdit = (record: Teacher) => {
    setEditingTeacher(record);
    const formValues = {
      ...record,
      certifications_input: record.certifications?.join(', ') || '',
      specialties_input: record.specialties?.join(', ') || '',
    };
    form.setFieldsValue(formValues);
    setModalVisible(true);
  };

  const handleDelete = async (id: number) => {
    try {
      await apiClient.delete(`/admin/teachers/${id}`);
      message.success('删除成功');
      fetchTeachers();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleUpload = async (file: any) => {
    setUploading(true);
    const formData = new FormData();
    formData.append('file', file);
    try {
      const response = await apiClient.post('/upload', formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      });

      const result = response.data;

      if (result.success) {
        form.setFieldValue('avatar_url', result.url);
        message.success('头像上传成功');
      } else {
        message.error(result.error || '上传失败');
      }
    } catch (error) {
      message.error('上传失败');
    } finally {
      setUploading(false);
    }

    return false; // Prevent default upload behavior
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      console.log('values', values)

      // Process certifications and specialties
      const processedValues = {
        ...values,
        certifications: values.certifications_input
          ? values.certifications_input.split(',').map((item: string) => item.trim()).filter(Boolean)
          : [],
        specialties: values.specialties_input
          ? values.specialties_input.split(',').map((item: string) => item.trim()).filter(Boolean)
          : [],
      };

      delete processedValues.certifications_input;
      delete processedValues.specialties_input;

      if (editingTeacher) {
        await apiClient.put(`/admin/teachers/${editingTeacher.id}`, processedValues);
        message.success('更新成功');
      } else {
        await apiClient.post('/admin/teachers', processedValues);
        message.success('创建成功');
      }

      setModalVisible(false);
      fetchTeachers();
    } catch (error) {
      message.error('操作失败');
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
      title: '头像',
      dataIndex: 'avatar_url',
      key: 'avatar_url',
      render: (avatar_url: string, record: Teacher) => (
        <Avatar
          src={avatar_url}
          icon={<UserOutlined />}
          size="large"
        />
      ),
    },
    {
      title: '姓名',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
      ellipsis: true,
    },
    {
      title: '专长',
      dataIndex: 'specialties',
      key: 'specialties',
      render: (specialties: string[]) => (
        <div>
          {specialties?.map((specialty, index) => (
            <Tag key={index} color="blue">{specialty}</Tag>
          ))}
        </div>
      ),
    },
    {
      title: '经验',
      dataIndex: 'experience_years',
      key: 'experience_years',
      render: (years: number) => `${years}年`,
      width: 80,
    },
    {
      title: '评分',
      dataIndex: 'average_rating',
      key: 'average_rating',
      render: (rating: number, record: Teacher) => (
        <div>
          <Rate disabled value={rating} allowHalf />
          <div className="text-xs text-gray-500">
            ({record.total_ratings || 0}条评价)
          </div>
        </div>
      ),
    },
    {
      title: '状态',
      dataIndex: 'is_active',
      key: 'is_active',
      render: (active: boolean) => (
        <Switch checked={active} disabled />
      ),
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record: Teacher) => (
        <Space>
          <Button
            type="primary"
            size="small"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title="确定要删除这位教师吗？"
            onConfirm={() => handleDelete(record.id)}
            okText="确定"
            cancelText="取消"
          >
            <Button
              type="primary"
              danger
              size="small"
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
        title="教师管理"
        extra={
          <Button
            type="primary"
            icon={<PlusOutlined />}
            onClick={handleCreate}
          >
            新增教师
          </Button>
        }
      >
        <Table
          columns={columns}
          dataSource={teachers}
          rowKey="id"
          loading={loading}
          pagination={{ pageSize: 10 }}
        />
      </Card>

      <Modal
        title={editingTeacher ? '编辑教师' : '新增教师'}
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            label="教师姓名"
            name="name"
            rules={[{ required: true, message: '请输入教师姓名' }]}
          >
            <Input placeholder="请输入教师姓名" />
          </Form.Item>

          <Form.Item
            label="头像"
            name="avatar_url"
          >
            <Input placeholder="头像图片链接" style={{ marginBottom: 8 }} />
            <Upload
              beforeUpload={handleUpload}
              showUploadList={false}
              accept="image/*"
              disabled={uploading}
            >
              <Button
                icon={<UploadOutlined />}
                loading={uploading}
                style={{ width: '100%' }}
              >
                {uploading ? '上传中...' : '上传头像'}
              </Button>
            </Upload>
          </Form.Item>

          <Form.Item
            label="简短描述"
            name="description"
          >
            <Input placeholder="请输入简短描述" />
          </Form.Item>

          <Form.Item
            label="详细简历"
            name="bio"
          >
            <TextArea rows={4} placeholder="请输入详细简历" />
          </Form.Item>

          <Form.Item
            label="认证资质"
            name="certifications_input"
            help="多个资质请用逗号分隔，例如：RYT-200, RYT-500"
          >
            <Input placeholder="请输入认证资质" />
          </Form.Item>

          <Form.Item
            label="专长领域"
            name="specialties_input"
            help="多个专长请用逗号分隔，例如：哈他瑜伽, 阴瑜伽"
          >
            <Input placeholder="请输入专长领域" />
          </Form.Item>

          <Form.Item
            label="教学经验年数"
            name="experience_years"
          >
            <InputNumber min={0} max={50} placeholder="请输入教学经验年数" className="w-full" />
          </Form.Item>

          <Form.Item
            label="是否激活"
            name="is_active"
            valuePropName="checked"
          >
            <Switch />
          </Form.Item>
        </Form>
      </Modal>
    </>
  );
}